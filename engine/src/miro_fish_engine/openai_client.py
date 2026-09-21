from __future__ import annotations

import os
from dataclasses import dataclass
from typing import Any, Type, TypeVar

from openai import AsyncOpenAI, OpenAIError
from pydantic import BaseModel
from tenacity import retry, retry_if_exception_type, stop_after_attempt, wait_exponential

T = TypeVar("T", bound=BaseModel)

DEFAULT_FAST_MODEL = "gpt-4o-mini"
DEFAULT_REASONING_MODEL = "gpt-4o"


@dataclass(frozen=True)
class LLMUsage:
    input_tokens: int
    output_tokens: int
    total_tokens: int
    model: str


@dataclass(frozen=True)
class LLMTurnResponse:
    content: str
    usage: LLMUsage


def is_reasoning_model(model: str) -> bool:
    """Returns True if the specified model is an OpenAI reasoning model (e.g. o1, o3-mini)."""
    normalized = model.lower()
    return normalized.startswith(("o1", "o3", "o-"))


class MiroFishOpenAIClient:
    """Production-grade async OpenAI client wrapper for Preflight Multi-Agent Engine.
    Handles authentication, exponential backoff retries, structured output parsing,
    reasoning model adaptation (o1/o3-mini parameter normalization), and accurate
    token usage accounting.
    """

    def __init__(self, api_key: str | None = None, base_url: str | None = None) -> None:
        self.api_key = api_key or os.environ.get("OPENAI_API_KEY")
        self.base_url = base_url or os.environ.get("OPENAI_BASE_URL")
        self._client: AsyncOpenAI | None = None

        if self.api_key:
            self._client = AsyncOpenAI(api_key=self.api_key, base_url=self.base_url)

    @property
    def is_configured(self) -> bool:
        return self._client is not None

    def _normalize_messages_for_model(
        self, messages: list[dict[str, str]], model: str
    ) -> list[dict[str, str]]:
        """Converts system messages to developer messages for reasoning models that require it."""
        if not is_reasoning_model(model):
            return messages

        normalized = []
        for msg in messages:
            if msg.get("role") == "system":
                normalized.append({"role": "developer", "content": msg["content"]})
            else:
                normalized.append(msg)
        return normalized

    @retry(
        retry=retry_if_exception_type(OpenAIError),
        wait=wait_exponential(multiplier=1, min=2, max=10),
        stop=stop_after_attempt(3),
        reraise=True,
    )
    async def parse_structured_output(
        self,
        *,
        response_format: Type[T],
        system_prompt: str,
        user_prompt: str,
        model: str = DEFAULT_FAST_MODEL,
        temperature: float = 0.7,
    ) -> tuple[T, LLMUsage]:
        """Parses OpenAI completion into a validated Pydantic model with Structured Outputs."""
        if not self._client:
            raise RuntimeError("OpenAI client is not configured. OPENAI_API_KEY is required.")

        raw_messages = [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": user_prompt},
        ]
        messages = self._normalize_messages_for_model(raw_messages, model)

        kwargs: dict[str, Any] = {
            "model": model,
            "messages": messages,
            "response_format": response_format,
        }
        if not is_reasoning_model(model):
            kwargs["temperature"] = temperature

        completion = await self._client.beta.chat.completions.parse(**kwargs)

        message = completion.choices[0].message
        parsed_data = message.parsed
        if parsed_data is None:
            err = message.refusal or "No structured content returned"
            raise ValueError(f"Failed to parse structured output from model response: {err}")

        usage = LLMUsage(
            input_tokens=completion.usage.prompt_tokens if completion.usage else 0,
            output_tokens=completion.usage.completion_tokens if completion.usage else 0,
            total_tokens=completion.usage.total_tokens if completion.usage else 0,
            model=model,
        )

        return parsed_data, usage

    @retry(
        retry=retry_if_exception_type(OpenAIError),
        wait=wait_exponential(multiplier=1, min=2, max=10),
        stop=stop_after_attempt(3),
        reraise=True,
    )
    async def generate_chat_turn(
        self,
        *,
        system_prompt: str,
        messages: list[dict[str, str]],
        model: str = DEFAULT_FAST_MODEL,
        temperature: float = 0.75,
        max_tokens: int = 600,
    ) -> LLMTurnResponse:
        """Generates a single agent debate turn with token usage tracking and
        reasoning model support.
        """
        if not self._client:
            raise RuntimeError("OpenAI client is not configured. OPENAI_API_KEY is required.")

        full_messages = [{"role": "system", "content": system_prompt}] + messages
        normalized_messages = self._normalize_messages_for_model(full_messages, model)

        kwargs: dict[str, Any] = {
            "model": model,
            "messages": normalized_messages,
        }

        if is_reasoning_model(model):
            kwargs["max_completion_tokens"] = max_tokens
        else:
            kwargs["temperature"] = temperature
            kwargs["max_tokens"] = max_tokens

        completion = await self._client.chat.completions.create(**kwargs)

        content = completion.choices[0].message.content or ""
        usage = LLMUsage(
            input_tokens=completion.usage.prompt_tokens if completion.usage else 0,
            output_tokens=completion.usage.completion_tokens if completion.usage else 0,
            total_tokens=completion.usage.total_tokens if completion.usage else 0,
            model=model,
        )

        return LLMTurnResponse(content=content.strip(), usage=usage)

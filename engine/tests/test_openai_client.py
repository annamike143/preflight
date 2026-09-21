from unittest.mock import AsyncMock, MagicMock

import pytest

from miro_fish_engine.openai_client import (
    LLMTurnResponse,
    LLMUsage,
    MiroFishOpenAIClient,
)
from miro_fish_engine.runtime_personas import (
    LLMExtractedPersona,
    LLMPersonaRoster,
    build_runtime_persona_foundation_with_openai,
    derive_seed_context,
)


def test_openai_client_unconfigured():
    client = MiroFishOpenAIClient(api_key=None)
    assert not client.is_configured


def test_openai_client_configured():
    client = MiroFishOpenAIClient(api_key="sk-test-mock-key-12345")
    assert client.is_configured
    assert client.api_key == "sk-test-mock-key-12345"


@pytest.mark.asyncio
async def test_openai_client_generate_turn_mocked():
    client = MiroFishOpenAIClient(api_key="sk-test-mock-key")
    
    mock_choice = MagicMock()
    mock_choice.message.content = "This proposal has severe unit economics risks."
    mock_usage = MagicMock()
    mock_usage.prompt_tokens = 120
    mock_usage.completion_tokens = 45
    mock_usage.total_tokens = 165

    mock_completion = MagicMock()
    mock_completion.choices = [mock_choice]
    mock_completion.usage = mock_usage

    mock_chat = MagicMock()
    mock_chat.completions.create = AsyncMock(return_value=mock_completion)
    client._client = MagicMock()
    client._client.chat = mock_chat

    response = await client.generate_chat_turn(
        system_prompt="You are a skeptic.",
        messages=[{"role": "user", "content": "What about costs?"}],
    )

    assert isinstance(response, LLMTurnResponse)
    assert "unit economics" in response.content
    assert response.usage.input_tokens == 120
    assert response.usage.output_tokens == 45
    assert response.usage.total_tokens == 165


@pytest.mark.asyncio
async def test_build_runtime_personas_with_mocked_openai():
    client = MiroFishOpenAIClient(api_key="sk-test-mock-key")

    mock_roster = LLMPersonaRoster(
        personas=[
            LLMExtractedPersona(
                label="Enterprise Buyer",
                perspective="favorable",
                focus_note="Assess procurement viability and compliance overhead.",
                critique_vector="Integration friction with legacy ERP.",
            ),
            LLMExtractedPersona(
                label="Security Auditor",
                perspective="skeptical",
                focus_note="Scrutinize local credential storage and telemetry leakage.",
                critique_vector="Vulnerability to unauthorized process injection.",
            ),
            LLMExtractedPersona(
                label="Financial Controller",
                perspective="analytical",
                focus_note="Evaluate gross margins and inference spend ratios.",
                critique_vector="Runaway token consumption on open-ended queries.",
            ),
        ]
    )

    client.parse_structured_output = AsyncMock(
        return_value=(
            mock_roster,
            LLMUsage(input_tokens=200, output_tokens=100, total_tokens=300, model="gpt-4o-mini"),
        )
    )

    seed = derive_seed_context(
        source_name="pitch_deck.pdf",
        source_kind="pdf",
        fingerprint_sha256="abcdef1234567890",
        raw_text="Enterprise SaaS architecture with BYOK multi-agent simulation capabilities.",
        participant_count=3,
    )

    foundation = await build_runtime_persona_foundation_with_openai(seed, client)
    assert foundation.moderator.role == "moderator"
    labels = foundation.stable_labels()
    assert "Enterprise Buyer" in labels
    assert "Security Auditor" in labels
    assert "Financial Controller" in labels
    assert set(foundation.perspective_coverage()) >= {"favorable", "skeptical", "analytical"}


@pytest.mark.asyncio
async def test_openai_client_reasoning_model_normalization():
    client = MiroFishOpenAIClient(api_key="sk-test-mock-key")

    mock_choice = MagicMock()
    mock_choice.message.content = "Adversarial critique from o3-mini."
    mock_usage = MagicMock()
    mock_usage.prompt_tokens = 250
    mock_usage.completion_tokens = 80
    mock_usage.total_tokens = 330

    mock_completion = MagicMock()
    mock_completion.choices = [mock_choice]
    mock_completion.usage = mock_usage

    mock_chat = MagicMock()
    mock_chat.completions.create = AsyncMock(return_value=mock_completion)
    client._client = MagicMock()
    client._client.chat = mock_chat

    response = await client.generate_chat_turn(
        system_prompt="You are an o3-mini security auditor.",
        messages=[{"role": "user", "content": "Critique this design."}],
        model="o3-mini",
        temperature=0.7,
        max_tokens=800,
    )

    assert response.content == "Adversarial critique from o3-mini."
    call_kwargs = mock_chat.completions.create.call_args.kwargs
    assert call_kwargs["model"] == "o3-mini"
    assert "temperature" not in call_kwargs
    assert call_kwargs["max_completion_tokens"] == 800
    assert "max_tokens" not in call_kwargs
    # Check system was normalized to developer
    messages = call_kwargs["messages"]
    assert messages[0]["role"] == "developer"
    assert messages[0]["content"] == "You are an o3-mini security auditor."


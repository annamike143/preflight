# RFC 402: Event-Driven Payment Webhook Ingestion Engine

## Status: Proposed
## Author: Architecture Team

## Summary
This document proposes replacing our polling-based payment verification architecture with an idempotent, event-driven webhook ingestion pipeline supporting Stripe, Adyen, and PayPal.

## Motivation
During flash-sale spikes, our polling worker pool exhausts database connection pools, increasing transaction settlement latency to over 14 seconds. Real-time webhooks eliminate polling overhead.

## Architecture & Failure Modes
1. **Ingress Gateway:** Edge terminates inbound HMAC-signed webhook payloads.
2. **Buffering Queue:** Kafka topic partitioned by merchant account ID.
3. **Worker Pool:** Processes events idempotently using Redis locks (TTL: 15s) and Postgres row-level locks.
4. **Dead Letter Queue (DLQ):** Messages failing after 5 retries are routed to SQS DLQ for manual inspection.

## Known Risks
- Network partitions between edge and Kafka broker during regional AWS degradation.
- Duplicate out-of-order webhook delivery during provider retry storms.
- Replay attacks if HMAC signing secret rotation is not coordinated zero-downtime.

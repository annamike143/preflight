from __future__ import annotations

import argparse
import sys
import time
from pathlib import Path

from .workspace_harness import HeadlessWorkspaceHarness


def format_duration(ms: int) -> str:
    seconds = ms / 1000.0
    return f"{seconds:.1f}s"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        prog="preflight",
        description="Preflight: Open-Source AI Multi-Agent Stress-Testing Engine (Headless CLI)",
    )
    parser.add_argument(
        "--seed",
        required=True,
        type=str,
        help="Path to the seed document (PDF, DOCX, Markdown, or TXT)",
    )
    parser.add_argument(
        "--output",
        "-o",
        type=str,
        default=None,
        help="Output path for the generated PDF report (default: ./preflight_report_<seed>.pdf)",
    )
    parser.add_argument(
        "--agents",
        "-a",
        type=int,
        default=4,
        help="Number of synthetic personas to extract and debate (default: 4)",
    )
    parser.add_argument(
        "--rounds",
        "-r",
        type=int,
        default=3,
        help="Number of moderated debate rounds (default: 3)",
    )
    parser.add_argument(
        "--budget",
        "-b",
        type=int,
        default=12000,
        help="Maximum token budget for the simulation (default: 12000)",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Validate seed document and print planned simulation parameters without running",
    )

    args = parser.parse_args(argv)

    seed_file = Path(args.seed).resolve()
    if not seed_file.exists():
        print(f"Error: Seed document not found at '{seed_file}'", file=sys.stderr)
        return 1

    print("================================================================================")
    print("        PREFLIGHT: AI MULTI-AGENT STRESS-TESTING & DILIGENCE ENGINE             ")
    print("================================================================================")
    print(f"  Target Seed Document: {seed_file.name} ({seed_file.stat().st_size} bytes)")
    print(f"  Synthetic Personas:   {args.agents} personas")
    print(f"  Moderated Rounds:     {args.rounds} rounds")
    print(f"  Max Token Budget:     {args.budget:,} tokens")

    if args.dry_run:
        print("\n[DRY RUN] Seed document validated successfully. Ready for simulation.")
        print("================================================================================")
        return 0

    print("\n[1/4] Bootstrapping isolated ephemeral simulation workspace...")
    start_time = time.perf_counter()

    try:
        harness = HeadlessWorkspaceHarness(
            seed_path=seed_file,
            agent_count=args.agents,
            round_count=args.rounds,
            token_budget=args.budget,
        )

        print("[2/4] Synthesizing orthogonal stakeholder personas...")
        print("[3/4] Orchestrating AI-moderated debate rounds...")
        result = harness.run(output_pdf_path=args.output)
        elapsed = time.perf_counter() - start_time

        print("[4/4] Compiling Executive Diligence PDF Report via ReportLab...")
        print("\n================================================================================")
        print("                         SIMULATION COMPLETE                                    ")
        print("================================================================================")
        print(f"  Personas Active:    {result.persona_count} ({', '.join(result.personas)})")
        print(f"  Debate Turns:       {result.turn_count} turns across {result.round_count} rounds")
        duration_str = format_duration(result.duration_ms)
        print(f"  Execution Time:     {duration_str} (Wall clock: {elapsed:.1f}s)")
        print(f"  Generated PDF:      {result.report_pdf_path}")
        print(f"  Structured Data:    {result.report_data_path}")
        print("================================================================================")
        return 0

    except Exception as exc:
        print(f"\nSimulation failed: {exc}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())

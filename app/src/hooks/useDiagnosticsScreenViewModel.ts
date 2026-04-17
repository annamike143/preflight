import {
  sections,
  type DiagnosticsScreenViewModel,
  type RunBoundedMemoryState,
  type RunLifecycleState,
  type RunModeratedRoundsState,
  type RunStatusState,
  type RunTokenDurationTrackingState,
  type RuntimePersonaState
} from "../appViewModels";
import { label } from "../appHelpers";

interface UseDiagnosticsScreenViewModelOptions {
  runtimePersonaState: RuntimePersonaState | null;
  runBoundedMemoryState: RunBoundedMemoryState | null;
  runModeratedRoundsState: RunModeratedRoundsState | null;
  runTokenDurationTrackingState: RunTokenDurationTrackingState | null;
  runLifecycleState: RunLifecycleState | null;
  runStatusState: RunStatusState | null;
}

export function useDiagnosticsScreenViewModel({
  runtimePersonaState,
  runBoundedMemoryState,
  runModeratedRoundsState,
  runTokenDurationTrackingState,
  runLifecycleState,
  runStatusState
}: UseDiagnosticsScreenViewModelOptions): DiagnosticsScreenViewModel {
  return {
    runtimePersonaState,
    runBoundedMemoryState,
    runModeratedRoundsState,
    runTokenDurationTrackingState,
    runLifecycleState,
    runStatusState,
    runtimePersonaContractLabel: runtimePersonaState?.contract_name ? label(runtimePersonaState.contract_name) : null,
    boundedMemorySurfaceLabel: runBoundedMemoryState?.memory_surface_name ? label(runBoundedMemoryState.memory_surface_name) : null,
    moderatedRoundsSurfaceLabel: runModeratedRoundsState?.surface_name ? label(runModeratedRoundsState.surface_name) : null,
    trackingSurfaceLabel: runTokenDurationTrackingState?.surface_name ? label(runTokenDurationTrackingState.surface_name) : null,
    lifecycleSurfaceLabel: runLifecycleState?.surface_name ? label(runLifecycleState.surface_name) : null,
    statusContractLabel: runStatusState?.contract_name ? label(runStatusState.contract_name) : null,
    sections
  };
}
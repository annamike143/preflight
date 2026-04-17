export const controlStyle = {
  borderRadius: 12,
  border: "1px solid #ccb89c",
  background: "#fffdf8",
  color: "#3d3125",
  padding: "11px 13px",
  fontSize: "0.95rem",
  boxSizing: "border-box" as const,
  width: "100%"
};

export function buttonStyle(kind: "primary" | "secondary", disabled = false) {
  const palette =
    kind === "primary"
      ? {
          border: "#4c3a21",
          background: "linear-gradient(180deg, #5f4526 0%, #4c3a21 100%)",
          color: "#fff7eb"
        }
      : {
          border: "#bda789",
          background: "rgba(255, 248, 238, 0.96)",
          color: "#4c3a21"
        };

  return {
    borderRadius: 999,
    border: `1px solid ${disabled ? "#d2c6b5" : palette.border}`,
    background: disabled ? "#ebe3d6" : palette.background,
    color: disabled ? "#8b7e6f" : palette.color,
    padding: "11px 17px",
    fontWeight: 700,
    letterSpacing: "0.01em",
    cursor: disabled ? "not-allowed" : "pointer",
    boxShadow: disabled ? "none" : "0 10px 22px rgba(76, 58, 33, 0.14)",
    transition: "transform 120ms ease, box-shadow 120ms ease"
  };
}

export const detailsStyle = {
  border: "1px solid #d9cfbf",
  borderRadius: 16,
  background: "rgba(255, 252, 246, 0.9)",
  padding: 14
};

export const detailsSummaryStyle = {
  cursor: "pointer",
  fontWeight: 700,
  color: "#4c3a21"
};
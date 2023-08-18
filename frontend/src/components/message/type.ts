export interface MessageProps {
  type: "success" | "error" | "info" | "warning";
  message: string;
  duration?: number;
}

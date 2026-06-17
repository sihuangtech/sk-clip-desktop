interface ErrorMessageProps {
  message: string;
}

export function ErrorMessage({ message }: ErrorMessageProps) {
  return (
    <div className="error-message">
      <span className="error-icon">⚠️</span>
      <span>{message}</span>
    </div>
  );
}

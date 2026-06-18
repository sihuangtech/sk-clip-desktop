import { AlertTriangle } from 'lucide-react';

interface ErrorMessageProps {
  message: string;
}

export function ErrorMessage({ message }: ErrorMessageProps) {
  return (
    <div className="error-message">
      <AlertTriangle className="error-icon" size={16} />
      <span>{message}</span>
    </div>
  );
}

import { CheckCircle2 } from 'lucide-react';

interface SuccessMessageProps {
  message: string;
}

export function SuccessMessage({ message }: SuccessMessageProps) {
  return (
    <div className="success-message">
      <CheckCircle2 className="success-icon" size={16} />
      <span>{message}</span>
    </div>
  );
}

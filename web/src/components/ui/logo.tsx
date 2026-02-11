import { Share2 } from "lucide-react";

export const Logo = ({ className }: { className?: string }) => {
  return (
    <div className={`relative flex items-center justify-center ${className}`}>
      <div className="bg-gradient-to-br from-blue-600 to-purple-600 rounded-lg p-1.5 shadow-lg">
        <Share2 className="w-5 h-5 text-white" strokeWidth={2.5} />
      </div>
    </div>
  );
};

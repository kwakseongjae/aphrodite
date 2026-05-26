"use client";

import { useState, ReactNode } from "react";
import { cn } from "./cn";

export interface CodeBlockProps {
  children: string;
  language?: string;
  copyable?: boolean;
  className?: string;
}

export function CodeBlock({ children, language, copyable = true, className }: CodeBlockProps) {
  const [copied, setCopied] = useState(false);
  const onCopy = async () => {
    try {
      await navigator.clipboard.writeText(children);
      setCopied(true);
      setTimeout(() => setCopied(false), 1200);
    } catch {
      // ignore
    }
  };
  return (
    <div className={cn("aph-codeblock-wrap", className)}>
      <pre className="aph-codeblock" data-language={language}>
        <code>{children}</code>
      </pre>
      {copyable && (
        <button type="button" className="aph-codeblock-copy" onClick={onCopy} aria-label={copied ? "복사됨" : "복사"}>
          {copied ? "복사됨" : "복사"}
        </button>
      )}
    </div>
  );
}

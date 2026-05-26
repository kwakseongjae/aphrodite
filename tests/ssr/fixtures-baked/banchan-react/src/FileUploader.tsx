"use client";

import { useState, useRef, ReactNode, DragEvent, ChangeEvent } from "react";
import { cn } from "./cn";

export interface FileUploaderProps {
  onFiles: (files: File[]) => void;
  accept?: string;
  multiple?: boolean;
  disabled?: boolean;
  className?: string;
  children?: ReactNode;
}

export function FileUploader({ onFiles, accept, multiple, disabled, className, children }: FileUploaderProps) {
  const inputRef = useRef<HTMLInputElement>(null);
  const [drag, setDrag] = useState(false);
  const handle = (files: FileList | null) => {
    if (!files || files.length === 0) return;
    onFiles(Array.from(files));
  };
  const onDrop = (e: DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    setDrag(false);
    handle(e.dataTransfer.files);
  };
  return (
    <div
      role="button"
      tabIndex={disabled ? -1 : 0}
      data-drag={drag}
      data-disabled={disabled || undefined}
      className={cn("aph-uploader", className)}
      onClick={() => !disabled && inputRef.current?.click()}
      onKeyDown={(e) => { if ((e.key === "Enter" || e.key === " ") && !disabled) inputRef.current?.click(); }}
      onDragOver={(e) => { e.preventDefault(); setDrag(true); }}
      onDragLeave={() => setDrag(false)}
      onDrop={onDrop}
    >
      <input
        ref={inputRef}
        type="file"
        accept={accept}
        multiple={multiple}
        style={{ display: "none" }}
        onChange={(e: ChangeEvent<HTMLInputElement>) => handle(e.target.files)}
      />
      {children ?? (
        <>
          <span style={{ fontSize: 24 }}>📎</span>
          <span><strong>파일을 끌어다 놓거나 클릭하여 업로드</strong></span>
        </>
      )}
    </div>
  );
}

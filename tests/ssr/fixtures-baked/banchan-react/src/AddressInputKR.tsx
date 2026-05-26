"use client";

import { useId } from "react";
import { cn } from "./cn";

export interface KoreanAddress {
  postcode: string;
  /** 도로명 또는 지번 주소. */
  road: string;
  /** 상세 주소 (동/호수/층 등). */
  detail: string;
}

export interface AddressInputKRProps {
  value: KoreanAddress;
  onChange: (next: KoreanAddress) => void;
  className?: string;
  /** Triggered when the user clicks the lookup button — wire to your
   *  Daum/Kakao postcode service. Receives `(setter)` so the lookup can
   *  push results back into the form. */
  onLookup?: (apply: (postcode: string, road: string) => void) => void;
}

export function AddressInputKR({ value, onChange, className, onLookup }: AddressInputKRProps) {
  const id = useId();
  const triggerLookup = () => {
    onLookup?.((postcode, road) => onChange({ ...value, postcode, road }));
  };
  return (
    <div className={cn("aph-address-kr", className)}>
      <div className="aph-address-kr__row">
        <input
          id={`${id}-postcode`}
          className="aph-input aph-address-kr__postcode"
          placeholder="우편번호"
          aria-label="우편번호"
          value={value.postcode}
          readOnly
        />
        <button type="button" className="aph-btn aph-btn--secondary" onClick={triggerLookup}>주소 검색</button>
      </div>
      <input
        className="aph-input"
        placeholder="도로명 주소"
        aria-label="도로명 주소"
        value={value.road}
        readOnly
      />
      <input
        className="aph-input"
        placeholder="상세 주소 (동/호수/층)"
        aria-label="상세 주소"
        value={value.detail}
        onChange={(e) => onChange({ ...value, detail: e.target.value })}
      />
    </div>
  );
}

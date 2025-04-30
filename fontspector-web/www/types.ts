
export type FontInfo = {
  name: string;
  file: Uint8Array;
  blob: any;
  face: any;
  font: any;
}

export type StatusCode = "FAIL" | "WARN" | "INFO" | "ERROR" | "PASS" | "SKIP";


interface ErrorMessage {
    error: string;
  }
  interface VersionMessage {
    version: string;
  }
  export interface ChecksMessage {
    checks: [string, Map<string, any>][];
  }
  export type Status = {
    message: string | null;
    severity: StatusCode;
    code: string | null;
    metadata: any | null;
  }
  export type CheckResult = {
    check_id: string;
    check_name: string;
    check_rationale: string;
    filename: string | null;
    section: string | null;
    subresults: Status[];
    worst_status: StatusCode;
  }
  
  export type Message = ErrorMessage | VersionMessage | ChecksMessage | CheckResult[];
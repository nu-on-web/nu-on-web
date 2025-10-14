import * as Monaco from "monaco-editor";
import type { Span } from "./types";
import lineColumn from "line-column";

export function spanToRange(content: string, span: Span): Monaco.Range {
  const finder = lineColumn(content);
  const start = finder.fromIndex(span.start);
  const end = finder.fromIndex(Math.min(content.length - 1, span.end));
  if (!start || !end) {
    return new Monaco.Range(0, 0, 0, 0);
  }
  return new Monaco.Range(start.line, start.col, end.line, end.col + 1);
}

export function moveSpanByOffset(span: Span, offset: number): Span {
  return {
    start: span.start + offset,
    end: span.end + offset,
  };
}

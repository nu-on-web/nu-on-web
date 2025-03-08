import * as Monaco from 'monaco-editor'
import type { Span } from "./types";
import lineColumn from 'line-column';

export function spanToRange(content: string, span: Span): Monaco.Range {
  const { line: startLineNumber, col: startColumn } = lineColumn(
    content,
  ).fromIndex(span.start)!;
  const { line: endLineNumber, col: endColumn } = lineColumn(
    content,
  ).fromIndex(
    Math.min(content.length - 1, span.end),
  )!;
  return new Monaco.Range(
    startLineNumber,
    startColumn,
    endLineNumber,
    endColumn + 1,
  );
}

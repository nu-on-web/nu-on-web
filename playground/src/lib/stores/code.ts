import { writable } from "svelte/store";

export const code = writable<string>("");

class HistoryStack {
  past: string[];
  future: string[];
  temp: string;
  viewingHistory: boolean;
  constructor() {
    this.past = []; // executed commands
    this.future = []; // undone commands (navigated forward)
    this.temp = ""; // current input before browsing history
    this.viewingHistory = false;
  }

  add(command: string) {
    if (command.trim() === "") return;
    this.past.push(command);
    this.future = []; // reset forward stack
    this.viewingHistory = false;
  }

  up(currentInput: string) {
    if (!this.viewingHistory) {
      this.temp = currentInput; // save current in-progress line
      this.viewingHistory = true;
    }

    if (this.past.length === 0) return currentInput;

    const cmd = this.past.pop();
    if (cmd) this.future.push(cmd);
    return cmd;
  }

  down() {
    if (this.future.length === 0) {
      if (this.viewingHistory) {
        this.viewingHistory = false;
        return this.temp; // restore what user was typing
      }
      return ""; // nothing to go forward to
    }

    const cmd = this.future.pop();
    if (cmd) this.past.push(cmd);
    return cmd;
  }
}

export const history = writable<HistoryStack>(new HistoryStack());

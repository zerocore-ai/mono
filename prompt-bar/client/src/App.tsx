import type { Component } from 'solid-js';
import { createEffect, onMount } from 'solid-js';
import styles from './App.module.css';
import { autofocus } from '@solid-primitives/autofocus';
import { useKeyDownEvent } from "@solid-primitives/keyboard";
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';


//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

const KEY_ESCAPE = "Escape";

const WINDOW_DID_BECOME_KEY = "window_did_become_key";

const WINDOW_DID_RESIGN_KEY = "window_did_resign_key";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const App: Component = () => {
  const event = useKeyDownEvent();
  let inputElementRef!: HTMLInputElement;

  createEffect(() => {
    const e = event();

    if (e?.key === KEY_ESCAPE) {
      console.log("Hiding window");
      invoke("hide_window");
      e.preventDefault();
    }
  });

  onMount(() => {
    listen(WINDOW_DID_BECOME_KEY, (_) => {
      inputElementRef.focus();
    });

    listen(WINDOW_DID_RESIGN_KEY, (_) => {
      inputElementRef.blur();
    });
  });

  return (
    <form data-tauri-drag-region>
      <input data-tauri-drag-region ref={inputElementRef} type="text" placeholder="Search" class={styles.input} />
    </form>
  );
};

export default App;

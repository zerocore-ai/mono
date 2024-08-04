import type { Component } from 'solid-js';
import {  onMount } from 'solid-js';

import styles from './App.module.css';
import { autofocus } from '@solid-primitives/autofocus';
// import { register } from "@tauri-apps/plugin-global-shortcut";

const App: Component = () => {
  onMount(async () => {
    const reg = async () => {
      // await register("Control+.", (event) => {
      //   if (event.state === "Pressed") {
      //     console.log("Shortcut triggered");
      //   }
      // });
    };

    reg();
  });

  return (
    <input ref={autofocus} autofocus type="text" placeholder="Search" class={styles.input} />
  );
};

export default App;

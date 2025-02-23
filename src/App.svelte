<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { listen } from '@tauri-apps/api/event';
  let layouts: any;

  onMount(async () => {

    listen("change_locale", (event) => {
      const window = getCurrentWindow();
      const languageElem = document.getElementById(`language-${event.payload}`);

      window.show();

      if (languageElem) {
        languageElem.style.backgroundColor = "#ffffff30";
      }

      setTimeout(async () => {
        await window.hide();

        if (languageElem) {
          languageElem.style.backgroundColor = "#ffffff00";
        }
      }, 500);
    });
    try {
      const layouts_local = await invoke('get_layouts');
      layouts = layouts_local;
    } catch (error) {
      console.error("Error fetching layouts:", error);
    }
  })
</script>

<div class="locales">
  {#each layouts as locale}
  <div class="language" id="language-{locale}">
    <h2 class="lang">{locale}</h2>
  </div>
  {/each}
</div>

<style>
  .locales{
    display: flex;
    height: 100vh;
    padding: 0;
    justify-content: space-between;
  }

  .language{
    padding: 0;
    /* background-color: #ffffff30; */
    height: 100%;
    align-items: center;

    display: flex;
    flex: 1;
  }

  .lang{
    padding: 0;
    margin: auto;
    font-size: 35px;
    text-align: center;
  }
</style>
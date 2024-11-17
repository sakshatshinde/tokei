<script lang="ts">
  import VideoPlayer from "../components/VideoPlayer.svelte";

  // let name = "";
  // let greetMsg = "";

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   greetMsg = await invoke("greet", { name });

  // let selectedTheme = "default";
  let videoSource: string = "D:/Videos/Sinderella/105-min-1-hi.mp4";
</script>

<VideoPlayer />

<script lang="ts">
  import { onMount } from "svelte";
  import { createChildWebview2 } from "$lib/utils";

  onMount(async () => {
    await createChildWebview2("anilist", "https://anilist.co/");
  });
</script>

<!-- <div class="hero h-full">
  <div class="hero-content text-center">
    <div class="max-w-2xl px-6">
      <div class="flex justify-center">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 512 512"
          width="200px"
          height="200px"
          shape-rendering="geometricPrecision"
          style="transform: scale(1); image-rendering: crisp-edges;"
        >
          <path
            d="M321.92 323.27V136.6c0-10.698-5.887-16.602-16.558-16.602h-36.433c-10.672 0-16.561 5.904-16.561 16.602v88.651c0 2.497 23.996 14.089 24.623 16.541 18.282 71.61 3.972 128.92-13.359 131.6 28.337 1.405 31.455 15.064 10.348 5.731 3.229-38.209 15.828-38.134 52.049-1.406.31.317 7.427 15.282 7.87 15.282h85.545c10.672 0 16.558-5.9 16.558-16.6v-36.524c0-10.698-5.886-16.602-16.558-16.602z"
            fill="#02a9ff"
          />
          <path
            d="M170.68 120 74.999 393h74.338l16.192-47.222h80.96L262.315 393h73.968l-95.314-273zm11.776 165.28 23.183-75.629 25.393 75.629z"
            fill="currentColor"
          />
        </svg>
      </div>
      <h1 class="text-6xl font-bold mb-6 tracking-tight">AniList launched</h1>
      <div class="backdrop-blur-sm p-4 mb-6 border">
        <h2 class="text-2xl font-medium tracking-wide">
          Please use the newly launched Anilist window
        </h2>
      </div>
    </div>
  </div>
</div> -->

<script lang="ts">
  interface Show {
    id: string; // Unique identifier for the show (e.g., a database ID or external API ID)
    title: string; // Title of the show
    Season: string;
    posterUrl: string; // URL of the show's poster image
    totalEpisodes: number; // Total number of episodes in the show
    lastWatchedEpisode: {
      episodeNumber: number | null; // Episode number last watched (null if none)
      timestamp: number | null; // Timestamp (milliseconds since epoch) of last watch
    };
  }

  const dummyShows: Show[] = [
    {
      id: "show1",
      title: "Toradora",
      Season: "1",
      posterUrl: "https://pic.re/image",
      totalEpisodes: 10,
      lastWatchedEpisode: {
        episodeNumber: 5,
        timestamp: 1678886400000, // Example timestamp (March 15, 2024)
      },
    },
    {
      id: "show2",
      title: "Attack on Titan",
      Season: "2",
      posterUrl:
        "https://platform.polygon.com/wp-content/uploads/sites/2/2024/08/filters_quality95formatwebp.webp",
      totalEpisodes: 87,
      lastWatchedEpisode: {
        episodeNumber: 1,
        timestamp: null,
      },
    },
  ];

  function resumeShow(show: Show) {
    // Implement your show resuming logic here.  This might involve
    // navigating to a specific episode in your app, sending an event
    // to your Tauri backend, etc.
    console.log(
      `Resuming show: ${show.title} (from episode ${show.lastWatchedEpisode || 1})`
    );
  }
</script>

<h1 class="text-2xl font-bold mb-4">Continue Watching</h1>

<div class="flex flex-wrap gap-4">
  {#each dummyShows as show}
    <div
      class="card max-w-80 max-h-80 shadow-xl glass hover:scale-105 will-change-transform transform transition duration-200"
    >
      <figure>
        <img src={show.posterUrl} alt={show.title} />
      </figure>
      <div class="card-body">
        <span class="card-title my-1 font-semibold">
          {show.title}
        </span>

        <div class="card-actions">
          <div class="flex items-center">
            <button
              class="btn btn-outline btn-sm mr-2"
              on:click={() => resumeShow(show)}
            >
              Resume
            </button>
            {#if show.lastWatchedEpisode.episodeNumber !== null}
              <div class="card-actions justify-end">
                <p class="mr-40"></p>
                <div class="badge badge-secondary p-2 justify-end">
                  S{show.Season} | EP{show.lastWatchedEpisode.episodeNumber + 1}
                </div>
              </div>
            {/if}
          </div>
        </div>
      </div>
    </div>
  {/each}
</div>

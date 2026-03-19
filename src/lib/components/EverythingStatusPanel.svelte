<script lang="ts">
  import { openPath, openUrl } from "@tauri-apps/plugin-opener";
  import type { EverythingStatus } from "$lib/types";

  interface Props {
    status: EverythingStatus | null;
    busy: string | null;
    onInstall: (packageId: string) => void;
    onCheck: () => void;
    checked: boolean;
  }

  let { status, busy, onInstall, onCheck, checked }: Props = $props();

  const downloadOptions = [
    { id: "setup-x86", label: "Run x86 installer", hint: "Auto-launch the 32-bit setup" },
    { id: "setup-x64", label: "Run x64 installer", hint: "Auto-launch the 64-bit setup" },
    { id: "zip-x86", label: "Download x86 ZIP", hint: "Grab the portable 32-bit archive" },
    { id: "zip-x64", label: "Download x64 ZIP", hint: "Grab the portable 64-bit archive" },
  ];

  function installToken(id: string) {
    return `everything-install-${id}`;
  }

  function openEsLocation() {
    if (status?.esPath) {
      void openPath(status.esPath);
    }
  }
</script>

<div class="panel rounded-[28px] p-4 md:p-5">
  <div class="flex flex-wrap items-center justify-between gap-3">
    <div>
      <p class="mono text-[11px] uppercase tracking-[0.24em] text-[color:var(--muted)]">Everything index</p>
      {#if checked}
        {#if status?.available}
          <p class="text-sm text-[color:var(--accent-2)]">es.exe detected: {status.esPath}</p>
          <p class="text-xs text-[color:var(--muted)]">Everything is ready; project scans will use the es.exe engine for instant results.</p>
        {:else}
          <p class="text-sm text-[color:var(--warn)]">es.exe not detected; falling back to slower filesystem scanning.</p>
          <p class="text-xs text-[color:var(--muted)]">Point Everything to PATH or download it to unlock full PC-wide scanning.</p>
        {/if}
      {:else}
        <p class="text-sm text-[color:var(--muted)]">Everything status has not been checked yet.</p>
        <p class="text-xs text-[color:var(--muted)]">Run the check when you are ready—this avoids auto-detecting es.exe while the app is still scanning other systems.</p>
      {/if}
    </div>
    <div class="flex flex-wrap gap-2">
      {#if checked && status?.available}
        <button
          class="rounded-full border border-white/10 bg-white/6 px-4 py-2 text-sm font-semibold transition hover:border-white/20 hover:bg-white/10"
          onclick={openEsLocation}
        >
          Reveal es.exe
        </button>
      {/if}
      <button
        class="rounded-full bg-[color:var(--accent)] px-4 py-2 text-sm font-semibold text-slate-950 transition hover:scale-[1.01]"
        onclick={() => openUrl("https://www.voidtools.com/downloads/")}
      >
        Download Everything
      </button>
    </div>
  </div>

  <div class="mt-3 flex flex-wrap items-center gap-3">
    <button
      class="rounded-full border border-white/10 bg-white/6 px-4 py-2 text-sm font-semibold transition hover:border-white/20 hover:bg-white/10 disabled:opacity-60"
      onclick={onCheck}
      disabled={busy === "everything-status"}
    >
      {busy === "everything-status" ? "Checking..." : checked ? "Re-check status" : "Check Everything status"}
    </button>
    <span class="text-xs text-[color:var(--muted)]">
      {checked ? "Status was last refreshed manually." : "Manual check skips the automatic scan loop."}
    </span>
  </div>

  <div class="mt-4 rounded-2xl border border-white/8 bg-white/4 p-4">
    <div class="flex items-center justify-between gap-2">
      <p class="text-xs uppercase tracking-[0.2em] text-[color:var(--muted)]">Auto install Everything</p>
      <span class="text-[11px] text-[color:var(--muted)]">Downloads land in %TEMP%</span>
    </div>
    <p class="mt-1 text-xs text-[color:var(--muted)]">The selected package will download, open, and (when possible) launch immediately.</p>

    <div class="mt-3 flex flex-wrap gap-2">
      {#each downloadOptions as option}
        <button
          class="flex flex-col rounded-2xl border border-white/10 bg-white/6 px-3 py-2 text-left text-sm font-semibold transition hover:border-white/20 hover:bg-white/10 disabled:opacity-60"
          onclick={() => onInstall(option.id)}
          disabled={busy === installToken(option.id)}
        >
          <span>{busy === installToken(option.id) ? "Downloading..." : option.label}</span>
          <span class="text-[10px] normal-case text-[color:var(--muted)]">{option.hint}</span>
        </button>
      {/each}
    </div>
  </div>
</div>

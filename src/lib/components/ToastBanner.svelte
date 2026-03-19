<script lang="ts">
  interface Toast {
    tone: "success" | "error";
    message: string;
  }

  interface Props {
    toast: Toast | null;
  }

  let { toast }: Props = $props();
</script>

{#if toast}
  <div class="fixed bottom-5 right-5">
    <div class={`panel rounded-2xl border px-4 py-3 text-sm ${toast.tone === "success" ? "border-[color:var(--accent)]/35" : "border-[color:var(--danger)]/35 text-[color:var(--danger)]"}`}>
      {toast.message}
    </div>
  </div>
{/if}

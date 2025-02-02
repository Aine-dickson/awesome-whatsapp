<template>
    <div>
        <div class="relative">
            <span>{{ message.sender }}</span>
            <div class="bg-slate-100 dark:bg-slate-950 rounded-lg p-2">
                {{ message.content }}
            </div>
            <div class="absolute -bottom-1.5 left-1.5 z-10">
                <img src="" alt="">
            </div>
        </div>
        <div class="flex justify-end text-sm font-extralight">
            <span>{{ message.sent_at }}</span>
        </div>
    </div>
</template>

<script lang="ts" setup>
let props = defineProps<{
    message: {
        sender: string,
        content: string,
        sent_at: string
    }
}>();
</script>
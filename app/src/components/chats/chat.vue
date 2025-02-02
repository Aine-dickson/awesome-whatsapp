<template>
    <div>
        <div class="w-12 h-12 rounded-full overflow-hidden">
            <img src="" alt="" class="w-full h-full object-cover">
        </div>
        <div class="flex flex-col flex-1">
            <span>{{ chat.name }}</span>
            <span v-if="chat.messages.length > 0" class="truncate">{{ chat.messages[chat.messages.length-1].content }}</span>
            <span v-else></span>
        </div>
    </div>
</template>

<script setup lang="ts">
let props = defineProps<{
    chat: {
        name: string,
        messages: {
            sender: string,
            content: string,
            sent_at: string
        }[]
    }
}>();
</script>
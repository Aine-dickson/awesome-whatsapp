<template>
    <div>
        <div>
            <img src="" alt="">
        </div>
        <span>{{ contact.name }}</span>
    </div>
</template>

<script setup lang="ts">
let props = defineProps<{
    contact: {
        name: string
    }
}>();
</script>
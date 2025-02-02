<template>
    <div class="h-full grid app">
        <div class="h-full sideBar">
            <sideBar/>
        </div>
        <div class="main h-full">
            <RouterView/>
        </div>
    </div>
</template>

<script lang="ts" setup>
    import sideBar from '@/components/side_bar.vue';
</script>

<style scoped>
    .app{
        grid-template-columns: min-content 1fr;
        grid-template-rows: 100%;
        grid-template-areas: "sideBar main";
    }

    .sideBar{
        grid-area: sideBar;
    }

    .main{
        grid-area: main;
    }
</style>
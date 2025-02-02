<template>
    <div class="h-full">
        <button data-drawer-target="logo-sidebar" data-drawer-toggle="logo-sidebar" aria-controls="logo-sidebar" type="button" class="inline-flex items-center p-2 mt-2 ms-3 text-sm text-gray-500 rounded-lg sm:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600">
            <span class="sr-only">Open sidebar</span>
            <svg class="w-6 h-6" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                <path clip-rule="evenodd" fill-rule="evenodd" d="M2 4.75A.75.75 0 012.75 4h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 4.75zm0 10.5a.75.75 0 01.75-.75h7.5a.75.75 0 010 1.5h-7.5a.75.75 0 01-.75-.75zM2 10a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 10z"></path>
            </svg>
        </button>

        <aside id="logo-sidebar" class="h-full" aria-label="Sidebar">
            <div class="sideBar h-full grid bg-gray-50 dark:bg-gray-800">
                <div class="nav-list grid h-full divide-y dark:divide-gray-600">
                    <ul class="upper-group space-y-2 font-medium p-3 px-2">
                        <li class="w-min">
                            <span @click="selectTab('chats')" class="cursor-pointer flex items-center p-2 rounded-lg dark:text-white" :class="{'activeSidebarTab': $route.name == 'chats', 'inactiveSidebarTab': $route.name != 'chats'}">
                                <svg class="w-6 h-6" :class="{'inactiveSidebarTabIcon': $route.name != 'chats', 'activeSidebarTabIcon': $route.name == 'chats'}" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                                    <path fill-rule="evenodd" d="M3 6a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v9a2 2 0 0 1-2 2h-6.616l-2.88 2.592C8.537 20.461 7 19.776 7 18.477V17H5a2 2 0 0 1-2-2V6Zm4 2a1 1 0 0 0 0 2h5a1 1 0 1 0 0-2H7Zm8 0a1 1 0 1 0 0 2h2a1 1 0 1 0 0-2h-2Zm-8 3a1 1 0 1 0 0 2h2a1 1 0 1 0 0-2H7Zm5 0a1 1 0 1 0 0 2h5a1 1 0 1 0 0-2h-5Z" clip-rule="evenodd"/>
                                </svg>
                            </span>
                        </li>
                        <li class="w-min">
                            <span @click="selectTab('status')" class="cursor-pointer flex items-center p-2 rounded-lg dark:text-white" :class="{'activeSidebarTab': $route.name == 'status', 'inactiveSidebarTab': $route.name != 'status'}">
                                <svg class="w-6 h-6" :class="{'inactiveSidebarTabIcon': $route.name != 'status', 'activeSidebarTabIcon': $route.name == 'status'}" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                                    <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3M3.22302 14C4.13247 18.008 7.71683 21 12 21c4.9706 0 9-4.0294 9-9 0-4.97056-4.0294-9-9-9-3.72916 0-6.92858 2.26806-8.29409 5.5M7 9H3V5"/>
                                </svg>
                            </span>
                        </li>
                        <li class="w-min">
                            <span @click="selectTab('calls')" class="cursor-pointer flex items-center p-2 rounded-lg dark:text-white" :class="{'activeSidebarTab': $route.name == 'calls', 'inactiveSidebarTab': $route.name != 'calls'}">
                                <svg class="w-6 h-6" :class="{'inactiveSidebarTabIcon': $route.name != 'calls', 'activeSidebarTabIcon': $route.name == 'calls'}" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                                    <path fill-rule="evenodd" d="M6 5a2 2 0 0 1 2-2h4.157a2 2 0 0 1 1.656.879L15.249 6H19a2 2 0 0 1 2 2v7a2 2 0 0 1-2 2v-5a3 3 0 0 0-3-3h-3.22l-1.14-1.682A3 3 0 0 0 9.157 6H6V5Z" clip-rule="evenodd"/>
                                    <path fill-rule="evenodd" d="M3 9a2 2 0 0 1 2-2h4.157a2 2 0 0 1 1.656.879L12.249 10H3V9Zm0 3v7a2 2 0 0 0 2 2h11a2 2 0 0 0 2-2v-7H3Z" clip-rule="evenodd"/>
                                </svg>
                            </span>
                        </li>
                        <li class="w-min">
                            <span @click="selectTab('search')" class="cursor-pointer flex items-center p-2 rounded-lg dark:text-white" :class="{'activeSidebarTab': $route.name == 'search', 'inactiveSidebarTab': $route.name != 'search'}">
                                <svg v-if="$route.name == 'search'" class="w-6 h-6 text-blue-500 dark:text-blue-600" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                                    <path d="M10 2a8 8 0 1 0 0 16 8 8 0 0 0 0-16Z"/>
                                    <path fill-rule="evenodd" d="M21.707 21.707a1 1 0 0 1-1.414 0l-3.5-3.5a1 1 0 0 1 1.414-1.414l3.5 3.5a1 1 0 0 1 0 1.414Z" clip-rule="evenodd"/>
                                </svg>
                                <svg v-else class="w-6 h-6 text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                                    <path stroke="currentColor" stroke-linecap="round" stroke-width="2" d="m21 21-3.5-3.5M17 10a7 7 0 1 1-14 0 7 7 0 0 1 14 0Z"/>
                                </svg>
                            </span>
                        </li>
                        <li class="w-min">
                            <span @click="selectTab('editor')" class="cursor-pointer flex items-center p-2 rounded-lg dark:text-white" :class="{'activeSidebarTab': $route.name == 'editor', 'inactiveSidebarTab': $route.name != 'editor'}">
                                <svg class="h-6 w-6" :class="{'inactiveSidebarTabIcon': $route.name != 'editor', 'activeSidebarTabIcon': $route.name == 'editor'}" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                                    <path fill-rule="evenodd" d="M8 7V2.221a2 2 0 0 0-.5.365L3.586 6.5a2 2 0 0 0-.365.5H8Zm2 0V2h7a2 2 0 0 1 2 2v.126a5.087 5.087 0 0 0-4.74 1.368v.001l-6.642 6.642a3 3 0 0 0-.82 1.532l-.74 3.692a3 3 0 0 0 3.53 3.53l3.694-.738a3 3 0 0 0 1.532-.82L19 15.149V20a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V9h5a2 2 0 0 0 2-2Z" clip-rule="evenodd"/>
                                    <path fill-rule="evenodd" d="M17.447 8.08a1.087 1.087 0 0 1 1.187.238l.002.001a1.088 1.088 0 0 1 0 1.539l-.377.377-1.54-1.542.373-.374.002-.001c.1-.102.22-.182.353-.237Zm-2.143 2.027-4.644 4.644-.385 1.924 1.925-.385 4.644-4.642-1.54-1.54Zm2.56-4.11a3.087 3.087 0 0 0-2.187.909l-6.645 6.645a1 1 0 0 0-.274.51l-.739 3.693a1 1 0 0 0 1.177 1.176l3.693-.738a1 1 0 0 0 .51-.274l6.65-6.646a3.088 3.088 0 0 0-2.185-5.275Z" clip-rule="evenodd"/>
                                </svg>
                            </span>
                        </li>
                        <li class="w-min">
                            <span @click="selectTab('channels')" class="cursor-pointer flex items-center p-2 rounded-lg dark:text-white" :class="{'activeSidebarTab': $route.name == 'channels', 'inactiveSidebarTab': $route.name != 'channels'}">
                                <svg class="w-6 h-6" :class="{'inactiveSidebarTabIcon': $route.name != 'channels', 'activeSidebarTabIcon': $route.name == 'channels'}" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                                    <path fill-rule="evenodd" d="M5 6a3 3 0 1 1 4 2.83V10a1 1 0 0 0 1 1h4a1 1 0 0 0 1-1V8.83a3.001 3.001 0 1 1 2 0V10a3 3 0 0 1-3 3h-1v2.17a3.001 3.001 0 1 1-2 0V13h-1a3 3 0 0 1-3-3V8.83A3.001 3.001 0 0 1 5 6Z" clip-rule="evenodd"/>
                                </svg>
                            </span>
                        </li>
                    </ul>
                    <ul class="lower-group space-y-2 font-medium p-3 px-2 h-min">
                        <li class="w-min">
                            <span @click="selectTab('settings')" class="cursor-pointer flex items-center p-2 rounded-lg dark:text-white" :class="{'activeSidebarTab': $route.name == 'settings', 'inactiveSidebarTab': $route.name != 'settings'}">
                                <svg class="w-6 h-6" :class="{'inactiveSidebarTabIcon': $route.name != 'settings', 'activeSidebarTabIcon': $route.name == 'settings'}" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                                    <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 13v-2a1 1 0 0 0-1-1h-.757l-.707-1.707.535-.536a1 1 0 0 0 0-1.414l-1.414-1.414a1 1 0 0 0-1.414 0l-.536.535L14 4.757V4a1 1 0 0 0-1-1h-2a1 1 0 0 0-1 1v.757l-1.707.707-.536-.535a1 1 0 0 0-1.414 0L4.929 6.343a1 1 0 0 0 0 1.414l.536.536L4.757 10H4a1 1 0 0 0-1 1v2a1 1 0 0 0 1 1h.757l.707 1.707-.535.536a1 1 0 0 0 0 1.414l1.414 1.414a1 1 0 0 0 1.414 0l.536-.535 1.707.707V20a1 1 0 0 0 1 1h2a1 1 0 0 0 1-1v-.757l1.707-.708.536.536a1 1 0 0 0 1.414 0l1.414-1.414a1 1 0 0 0 0-1.414l-.535-.536.707-1.707H20a1 1 0 0 0 1-1Z"/>
                                    <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z"/>
                                </svg>
                            </span>
                        </li>
                        <li class="w-min">
                            <span @click="selectTab('profile')" class="cursor-pointer flex items-center p-2 rounded-lg dark:text-white" :class="{'activeSidebarTab': $route.name == 'profile', 'inactiveSidebarTab': $route.name != 'profile'}">
                                <svg  class="w-6 h-6" :class="{'inactiveSidebarTabIcon': $route.name != 'profile', 'activeSidebarTabIcon': $route.name == 'profile'}" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                                    <path fill-rule="evenodd" d="M12 20a7.966 7.966 0 0 1-5.002-1.756l.002.001v-.683c0-1.794 1.492-3.25 3.333-3.25h3.334c1.84 0 3.333 1.456 3.333 3.25v.683A7.966 7.966 0 0 1 12 20ZM2 12C2 6.477 6.477 2 12 2s10 4.477 10 10c0 5.5-4.44 9.963-9.932 10h-.138C6.438 21.962 2 17.5 2 12Zm10-5c-1.84 0-3.333 1.455-3.333 3.25S10.159 13.5 12 13.5c1.84 0 3.333-1.455 3.333-3.25S13.841 7 12 7Z" clip-rule="evenodd"/>
                                </svg>
                            </span>
                        </li>
                        <li class="w-min">
                            <span  @click="accountStore.toggle_theme" class="cursor-pointer flex items-center p-2 rounded-lg dark:text-white" :class="{'activeSidebarTab': $route.name == 'theme', 'inactiveSidebarTab': $route.name != 'theme'}">
                                <svg v-if="accountStore.theme == 'light'" class="w-6 h-6 text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                                    <path fill-rule="evenodd" d="M13 3a1 1 0 1 0-2 0v2a1 1 0 1 0 2 0V3ZM6.343 4.929A1 1 0 0 0 4.93 6.343l1.414 1.414a1 1 0 0 0 1.414-1.414L6.343 4.929Zm12.728 1.414a1 1 0 0 0-1.414-1.414l-1.414 1.414a1 1 0 0 0 1.414 1.414l1.414-1.414ZM12 7a5 5 0 1 0 0 10 5 5 0 0 0 0-10Zm-9 4a1 1 0 1 0 0 2h2a1 1 0 1 0 0-2H3Zm16 0a1 1 0 1 0 0 2h2a1 1 0 1 0 0-2h-2ZM7.757 17.657a1 1 0 1 0-1.414-1.414l-1.414 1.414a1 1 0 1 0 1.414 1.414l1.414-1.414Zm9.9-1.414a1 1 0 0 0-1.414 1.414l1.414 1.414a1 1 0 0 0 1.414-1.414l-1.414-1.414ZM13 19a1 1 0 1 0-2 0v2a1 1 0 1 0 2 0v-2Z" clip-rule="evenodd"/>
                                </svg>
                                <svg v-else class="w-6 h-6 text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                                    <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 5V3m0 18v-2M7.05 7.05 5.636 5.636m12.728 12.728L16.95 16.95M5 12H3m18 0h-2M7.05 16.95l-1.414 1.414M18.364 5.636 16.95 7.05M16 12a4 4 0 1 1-8 0 4 4 0 0 1 8 0Z"/>
                                </svg>
                            </span>
                        </li>
                    </ul>
                </div>
            </div>
        </aside>
    </div>
</template>

<script setup lang="ts">
    import { useAccountStore } from '@/stores/account';
    import { computed } from 'vue';
    import { useRouter } from 'vue-router';

    const accountStore = useAccountStore()
    const router = useRouter()

    const activeTab = computed(() => accountStore.active_nav)

    const selectTab = (target: string)=> {
        accountStore.activateNav(target)
        router.push({name: target})
    }
</script>

<style scoped>
@reference "../assets/tailwind.css";
.activeSidebarTab {
    @apply bg-gray-300 dark:bg-gray-700
}
.activeSidebarTabIcon {
    @apply text-blue-500 dark:text-blue-600
}
.inactiveSidebarTab {
    @apply hover:bg-gray-300 dark:hover:bg-gray-700
}
.inactiveSidebarTabIcon {
    @apply text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white
}
.upper-group{
    grid-area: upper;
}
.lower-group{
    grid-area: lower;
}
.nav-list{
    grid-area: list;
    grid-template-columns: 100%;
    grid-template-rows: 1fr min-content;
    grid-template-areas: "upper" "lower";
}
.app-title{
    grid-area: title;
}
.sideBar{
    grid-template-areas: "title" "list";
    grid-template-columns: 100%;
    grid-template-rows: min-content 1fr;
}
</style>
<template>
    <section class="h-full flex flex-col items-center justify-center px-6 py-8 lg:py-0">
        <div>
            <h1 class="mb-1 text-xl font-bold leading-tight tracking-tight text-gray-900 md:text-2xl dark:text-white">
                Comfirm your contact
            </h1>
            <p class="font-light text-gray-500 dark:text-gray-400">A six digit code has been sent to **********924</p>
            <form @submit.prevent="submit" @reset.prevent="accountStore.resend_verification" class="mt-4 space-y-10 lg:mt-5 md:space-y-5" action="#">
                <div class="flex mb-2 space-x-2 rtl:space-x-reverse">
                    <div>
                        <label for="code-1" class="sr-only">First code</label>
                        <input v-model="code1" type="text" maxlength="1" data-focus-input-init data-focus-input-next="code-2" id="code-1" class="block w-9 h-9 py-3 text-sm font-extrabold text-center text-gray-900 bg-white border border-gray-300 rounded-lg focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" required />
                    </div>
                    <div>
                        <label for="code-2" class="sr-only">Second code</label>
                        <input v-model="code2" type="text" maxlength="1" data-focus-input-init data-focus-input-prev="code-1" data-focus-input-next="code-3" id="code-2" class="block w-9 h-9 py-3 text-sm font-extrabold text-center text-gray-900 bg-white border border-gray-300 rounded-lg focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" required />
                    </div>
                    <div>
                        <label for="code-3" class="sr-only">Third code</label>
                        <input v-model="code3" type="text" maxlength="1" data-focus-input-init data-focus-input-prev="code-2" data-focus-input-next="code-4" id="code-3" class="block w-9 h-9 py-3 text-sm font-extrabold text-center text-gray-900 bg-white border border-gray-300 rounded-lg focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" required />
                    </div>
                    <div>
                        <label for="code-4" class="sr-only">Fourth code</label>
                        <input v-model="code4" type="text" maxlength="1" data-focus-input-init data-focus-input-prev="code-3" data-focus-input-next="code-5" id="code-4" class="block w-9 h-9 py-3 text-sm font-extrabold text-center text-gray-900 bg-white border border-gray-300 rounded-lg focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" required />
                    </div>
                    <div>
                        <label for="code-5" class="sr-only">Fifth code</label>
                        <input v-model="code5" type="text" maxlength="1" data-focus-input-init data-focus-input-prev="code-4" data-focus-input-next="code-6" id="code-5" class="block w-9 h-9 py-3 text-sm font-extrabold text-center text-gray-900 bg-white border border-gray-300 rounded-lg focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" required />
                    </div>
                    <div>
                        <label for="code-6" class="sr-only">Sixth code</label>
                        <input v-model="code6" type="text" maxlength="1" data-focus-input-init data-focus-input-prev="code-5" id="code-6" class="block w-9 h-9 py-3 text-sm font-extrabold text-center text-gray-900 bg-white border border-gray-300 rounded-lg focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" required />
                    </div>
                </div>

                <div class="flex items-start">
                    <div class="flex items-center h-5">
                        <input id="terms" aria-describedby="terms" type="checkbox" class="w-4 h-4 border border-gray-300 rounded bg-gray-50 focus:ring-3 focus:ring-blue-300 dark:bg-gray-700 dark:border-gray-600 dark:focus:ring-blue-600 dark:ring-offset-gray-800" required>
                    </div>
                    <div class="ml-3 text-sm">
                        <label for="terms" class="font-light text-gray-500 dark:text-gray-300">I accept the <a class="font-medium text-blue-600 hover:underline dark:text-blue-500" href="#">Terms and Conditions</a></label>
                    </div>
                </div>

                <div class="space-y-2">
                    <div class="w-full flex items-center justify-center space-x-2 text-white bg-blue-600 hover:bg-blue-700 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
                        <svg class="w-4 h-4" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.651 7.65a7.131 7.131 0 0 0-12.68 3.15M18.001 4v4h-4m-7.652 8.35a7.13 7.13 0 0 0 12.68-3.15M6 20v-4h4"/>
                        </svg>
                        <button type="reset" class="">Resend code</button>
                    </div>
                    <button type="submit" class="w-full text-white bg-blue-600 hover:bg-blue-700 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">Submit</button>
                </div>
            </form>
        </div>
    </section>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useAccountStore } from '@/stores/account';
import { useRouter } from 'vue-router';

let accountStore = useAccountStore();
let router = useRouter();

let code1 = ref('');
let code2 = ref('');
let code3 = ref('');
let code4 = ref('');
let code5 = ref('');
let code6 = ref('');

// use this simple function to automatically focus on the next input
function focusNextInput(el: HTMLInputElement, prevId: string, nextId: string) {
    if (el.value.length === 0) {
        if (prevId) {
            document.getElementById(prevId)?.focus();
        }
    } else {
        if (nextId) {
            document.getElementById(nextId)?.focus();
        }
    }
}

let submit = ()=> {
    let code = '';
    accountStore.verify_contact(code).then((response) => {
        if (response) {
            router.push({name: 'chats'});
        }
    });
}

onMounted(()=> {
    document.querySelectorAll('[data-focus-input-init]').forEach(element => {
        element.addEventListener('keyup', function(this: HTMLInputElement) {
            const prevId = this.getAttribute('data-focus-input-prev');
            const nextId = this.getAttribute('data-focus-input-next');
            focusNextInput(this, prevId || '', nextId || '');
        });
        
    // Handle paste event to split the pasted code into each input
        element.addEventListener('paste', function(this: HTMLInputElement, event: Event) {
            const clipboardEvent = event as ClipboardEvent;
            clipboardEvent.preventDefault();
            const pasteData = (clipboardEvent.clipboardData as DataTransfer).getData('text');
            const digits = pasteData.replace(/\D/g, ''); // Only take numbers from the pasted data
    
            // Get all input fields
            const inputs = document.querySelectorAll('[data-focus-input-init]');
            
            // Iterate over the inputs and assign values from the pasted string
            inputs.forEach((input, index) => {
                const inputElement = input as HTMLInputElement;
                if (digits[index]) {
                    inputElement.value = digits[index];
                    // Focus the next input after filling the current one
                    const nextId = inputElement.getAttribute('data-focus-input-next');
                    if (nextId) {
                        document.getElementById(nextId)?.focus();
                    }
                }
            });
        });
    });
})


</script>
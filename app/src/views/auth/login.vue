<template>
    <section class="flex flex-col items-center justify-center px-6 py-8 lg:py-0">
        <div class="w-[80%] md:mt-0 xl:p-0">
            <div class="p-6 space-y-4 md:space-y-10 sm:p-8">
                <div class="space-y-3">
                    <h1 class=" font-bold leading-tight tracking-tight text-gray-900 md:text-3xl dark:text-white">
                        Welcome back
                    </h1>
                    <span>Do not have an account? <span @click="$router.push({name: 'signup'})" class="underline underline-offset-2 cursor-pointer text-blue-500">Signup</span></span>
                </div>
                
                <form @submit.prevent="submit" class="space-y-12 mt-4" action="#">
                    <div class="space-y-6">
                        <div>
                            <label for="tel_contact" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Tel Contact</label>
                            <input type="tel" name="tel_contact" id="tel_contact" class="bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="256777777777" required>
                        </div>

                        <div class="relative border-b dark:border-gray-600">
                            <span class="absolute left-[50%] -top-3 px-2 bg-slate-200 dark:bg-slate-900">Or</span>
                        </div>

                        <div>
                            <label for="email" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Your email</label>
                            <input type="email" name="email" id="email" class="bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="name@company.com" required>
                        </div>
                    </div>

                    <div class="space-y-6">
                        <div>
                            <div class="flex justify-between">
                                <label for="passcode" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Passcode</label>
                                <spam @click="$router.push({name: 'account_recovery'})" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white cursor-pointer">Forgot your passcode?</spam>
                            </div>
                            <input type="password" name="passcode" id="passcode" class="bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="••••••••" required>
                        </div>
                        <button type="submit" class="w-full text-white bg-blue-600 hover:bg-blue-700 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">Next</button>
                    </div>
                    
                </form>
            </div>
        </div>
    </section>
</template>

<script setup lang="ts">
import { type User, useAccountStore } from '@/stores/account';
import { ref } from 'vue';
import { useRouter } from 'vue-router';

    let accountStore = useAccountStore();
    let router = useRouter();
    let passcode = ref('');
    let contact = ref('');

    const submit = async () => {
        let data: User = {
            passcode: passcode.value,
            tel_contact: contact.value,
        }

        accountStore.login(data).then((response) => {
            if (response) {
                router.push({name: 'chats'});
            }
        });
    }
</script>

<style scoped>
#country::-webkit-inner-spin-button,
#country::-webkit-outer-spin-button {
    -webkit-appearance: none;
    margin: 0;
}

#country {
    -moz-appearance: textfield;
}
</style>
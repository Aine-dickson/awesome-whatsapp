<template>
    <section class="flex flex-col items-center justify-center px-6 py-8 lg:py-0">
        <div class="w-[80%] md:mt-0 xl:p-0 dark:bg-slate-800 dark:border-slate-700">
            <div class="p-6 space-y-4 md:space-y-10 sm:p-8">
                <div class="space-y-3">
                    <h1 class=" font-bold leading-tight tracking-tight text-gray-900 md:text-3xl dark:text-white">
                        Create an account
                    </h1>
                    <span>Already have an account? <span @click="$router.push({name: 'login'})" class="underline underline-offset-2 cursor-pointer text-blue-500">Login</span></span>
                </div>
                <div class="">
                    <p>Wassap will need to verify your phone number.</p>
                    <p>Carrier charges may apply.</p>
                </div>
                <form @submit.prevent="submit" class="space-y-10 mt-4" action="#">
                    <div class="flex space-x-1">
                        <div class="flex w-[2.5rem] border-b border-gray-400">
                            <span>+</span>
                            <input type="number" min="1" max="999" name="country-code" id="country" class="w-full focus:outline-0" placeholder="257" required>
                        </div>
                        <div class="border-b border-gray-400 flex-1">
                            <input v-model="contact" type="tel" name="tel-contact" id="country" class="w-full focus:outline-0" placeholder="777777777" required>
                        </div>
                    </div>
                    <div class="flex space-x-2">
                        <div class="border-b border-gray-400">
                            <label for="">Four digit Passcode</label>
                            <input v-model="passcode" type="password" name="passcode" id="passcode" class="w-full focus:outline-0" placeholder="••••••••" required>
                        </div>
                        <div class="border-b border-gray-400">
                            <label for="">Comfirm Passcode</label>
                            <input type="password" name="passcode" id="passcode" class="w-full focus:outline-0" placeholder="••••••••" required>
                        </div>
                    </div>
                    
                    <button type="submit" class="w-full text-white bg-blue-600 hover:bg-blue-700 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-full text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">Next</button>
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

        accountStore.signup(data).then((response) => {
            if (response) {
                router.push({name: 'verify_contact'});
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
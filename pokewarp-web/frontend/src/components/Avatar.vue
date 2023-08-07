<script setup lang="ts">
import { useFetch } from '@vueuse/core';
import { Ref, ref } from 'vue';

const username: Ref<string> = ref("")
const password: Ref<string> = ref("")

async function login() {
    const formData = new FormData();

    formData.append("name", username.value);
    formData.append("password", password.value);

    const { statusCode } = await useFetch('/login', {
        body: JSON.stringify({
            "name": username.value,
            "password": password.value
        }),
        headers: {
            "Content-Type": "application/json"
        }
    }).post()

    if (statusCode.value != 200) {
        alert("An error occurred while logging in")
    }
}
</script>

<template>
    <div class="dropdown dropdown-top flex justify-center">
        <div tabindex="0" class="bg-white h-16 p-1 rounded-full flex items-center gap-3 px-3 shadow-md m-1 btn">
            <img src="https://archives.bulbagarden.net/media/upload/7/78/BW_HilbertOD_Stand.png" class="pixelated h-full" />
            <p class="font-semibold">TATIS</p>
        </div>
        <div tabindex="0" class="dropdown-content z-[1] shadow bg-base-100 rounded-box border p-3">
            <form action="" class="flex justify-center flex-col gap-3">
                <p class="font-semibold">Name</p>
                <input type="text" name="name" id="" class="input input-bordered" v-model="username">
                <p class="font-semibold">Password</p>
                <input type="password" name="password" id="" class="input input-bordered" v-model="password">
                <button type="button" value="Login" class="btn" @click="login" >Login</button>
            </form>
        </div>
    </div>
</template>
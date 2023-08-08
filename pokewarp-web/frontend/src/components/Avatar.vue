<script setup lang="ts">
import { Ref, ref } from 'vue';
import { db, isLoggedIn } from '../data';
import { useCookies } from '@vueuse/integrations/useCookies'
import { UserIcon } from '@heroicons/vue/24/solid'


const username: Ref<string> = ref("")
const password: Ref<string> = ref("")
const isLogin = ref(true)
const cookies = useCookies(['jwt'])


// We set the global ref depending on the jwt


isLoggedIn.value = !!cookies.get('jwt')

async function login() {
    let jwt = await db.signin({
        NS: "dev",
        DB: "pokewarp",
        SC: "allusers",

        name: username.value,
        pass: password.value
    }).catch(err => {
        alert(err)
    })

    cookies.set('jwt', jwt, {
        maxAge: 60 * 60 * 24 * 14
    })

    isLoggedIn.value = !!cookies.get('jwt')

}

async function signup() {
    db.signup({
        NS: "dev",
        DB: "pokewarp",
        SC: "allusers",

        name: "Tatis",
        pass: "tatis2007"
    })
}
</script>

<template>
    <div class="dropdown dropdown-top flex justify-center">
        <div tabindex="0" class="bg-white h-16 p-1 rounded-full flex items-center gap-3 px-3 shadow-md m-1 btn">
            <div v-if="isLoggedIn" class="flex items-center gap-3 h-full">
                <img src="https://archives.bulbagarden.net/media/upload/7/78/BW_HilbertOD_Stand.png"
                    class="pixelated h-full" />
                <p class="font-semibold">TATIS</p>
            </div>
            <div v-else class="flex items-center gap-3">
                <UserIcon class="w-8" />
                Login
            </div>
        </div>
        <div tabindex="0" class="dropdown-content z-[1] shadow bg-base-100 rounded-box border p-3">
            <div>
                <form action="" class="flex justify-center flex-col gap-3" v-if="isLogin">
                    <p class="font-semibold">Name</p>
                    <input type="text" name="name" id="" class="input input-bordered" v-model="username">
                    <p class="font-semibold">Password</p>
                    <input type="password" name="password" id="" class="input input-bordered" v-model="password">
                    <button class="underline" type="button" @click="isLogin = !isLogin">No account yet?</button>
                    <button type="button" value="Login" class="btn" @click="login">Login</button>
                </form>
                <form action="" class="flex justify-center flex-col gap-3" v-else>
                    <p class="font-semibold">Name</p>
                    <input type="text" name="name" id="" class="input input-bordered" v-model="username">
                    <p class="font-semibold">Password</p>
                    <input type="password" name="password" id="" class="input input-bordered" v-model="password">
                    <button class="underline" type="button" @click="isLogin = !isLogin">Already have an account?</button>
                    <button type="button" value="Login" class="btn" @click="signup">Signup</button>
                </form>
            </div>
        </div>
    </div>
</template>
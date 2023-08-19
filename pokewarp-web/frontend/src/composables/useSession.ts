import { ref } from "vue";
import { db } from "../data";
import { useCookies } from '@vueuse/integrations/useCookies.mjs';

const cookies = useCookies(['jwt'])

export function useSession() {
    const isLoggedIn = ref(false)
    const jwt = cookies.get<string>("jwt")

    function authenticateDb() {
        db.authenticate(jwt)
            .then(() => isLoggedIn.value = true)
            .catch(() => isLoggedIn.value = false)
    }

    function update() {
        if(jwt != "") {
            isLoggedIn.value = true
        } else {
            isLoggedIn.value = false
        }
    
        authenticateDb()
    }

    async function login(username: string, password: string) {
        let jwt = await db.signin({
            NS: "dev",
            DB: "pokewarp",
            SC: "allusers",
    
            name: username,
            pass: password
        }).catch(err => {
            alert(err)
        })
    
        cookies.set('jwt', jwt, {
            maxAge: 60 * 60 * 24 * 14
        })
    
        update()
    
    }
    
    async function signup(username: string, password: string) {
        db.signup({
            NS: "dev",
            DB: "pokewarp",
            SC: "allusers",
    
            name: username,
            pass: password
        })
    }

    update()

    return { isLoggedIn, jwt, login, signup }

}
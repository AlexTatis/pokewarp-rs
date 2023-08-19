import { Surreal } from "surrealdb.js"

export const db = new Surreal('http://127.0.0.1:8000/rpc', {
        ns: "dev",
        db: "pokewarp",
})
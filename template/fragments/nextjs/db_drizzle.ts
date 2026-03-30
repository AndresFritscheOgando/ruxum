import { drizzle } from "drizzle-orm/{{drizzle_provider}}";
{{import_db_client}}
import * as schema from "../../drizzle/schema";

{{db_client_init}}
export const db = drizzle(client, {{drizzle_options}});

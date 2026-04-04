import NextAuth from "next-auth";
import GitHub from "next-auth/providers/github";
import { env } from "./env";

export const { handlers, signIn, signOut, auth } = NextAuth({
  providers: [
    GitHub({
      clientId: env.AUTH_GITHUB_ID,
      clientSecret: env.AUTH_GITHUB_SECRET,
    }),
  ],
  secret: env.AUTH_SECRET,
  callbacks: {
    authorized({ auth }) {
      return !!auth?.user;
    },
  },
});

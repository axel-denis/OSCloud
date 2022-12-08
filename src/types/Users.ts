export type User = {
  id: number,
  name: string,
  type: "admin" | "user",
  password: string,
}
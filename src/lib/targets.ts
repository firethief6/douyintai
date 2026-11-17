import { call } from "./api";

export type TargetType = "user" | "video" | "live";

export interface TargetCard {
  type: TargetType;
  target_key: string;
  title: string;
  subtitle: string;
  image: string;
  uid?: string;
  aweme_id?: string;
  room_id?: string;
  kind?: "video" | "gallery" | "live";
  author?: { uid?: string; nickname?: string; avatar?: string };
}

export const searchTargets = async (q: string) =>
  (await call<{ items: TargetCard[] }>("/targets/search", { q, kinds: "user,video" })).items;

export const resolveTarget = async (target: string) =>
  (await call<{ item: TargetCard }>("/targets/resolve", { target })).item;

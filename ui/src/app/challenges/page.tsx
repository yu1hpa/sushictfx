import ChallengeCard from "@/components/ChallengeCard/ChallengeCard";
import { Challenge } from "../types";
import styles from "./page.module.scss";

async function getChallenges() {
  const res = await fetch("http://localhost:3000/api/challenges");

  if (!res.ok) {
    throw new Error("Failed to fetch challenges");
  }

  const data = await res.json();
  return data.challenges as Challenge[];
}

export default async function Challenges() {
  const challenges = await getChallenges();
  return (
    <>
      <h2>Challenges</h2>
      <div className={styles.challenge}>
        {challenges.map((challenge) => (
          <ChallengeCard key={challenge.id} challenge={challenge} />
        ))}
      </div>
    </>
  );
}

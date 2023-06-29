import type { Challenge } from "@/app/types";
import styles from "./ChallengeCard.module.scss";

export default function ChallengeCard({
  challenge,
}: {
  challenge: Challenge;
}): JSX.Element {
  return (
    <button className={styles.card}>
      <div className={styles.wrap__card_title}>
        <h6 className={styles.card_title}>{challenge.name}</h6>
      </div>
      <div className={styles.card_tags}>
        <div>
          {challenge.tags.map((tag) => (
            <span className={styles.card_tags_unit} key={challenge.id}>
              {tag}
            </span>
          ))}
        </div>
      </div>
      <div className={styles.wrap__points}>
        <p>{challenge.points} points</p>
      </div>
      <div className={styles.wrap__solved}>
        <p>{challenge.solved} solved</p>
      </div>
    </button>
  );
}

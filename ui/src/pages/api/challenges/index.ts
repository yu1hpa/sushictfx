import { randomUUID } from "crypto";
import fs from "fs";
import type { NextApiRequest, NextApiResponse } from "next";

const delay = (ms: number) => new Promise((res) => setTimeout(res, ms));

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  if (req.method === "GET") {
    await delay(1500);
    const challenges = JSON.parse(
      fs.readFileSync("sample/challenges.json", "utf8")
    );
    challenges.challenges.sort((a: any, b: any) => {
      return new Date(b.createdAt).valueOf() - new Date(a.createdAt).valueOf();
    });
    res.status(200).json(challenges);
  } else if (req.method === "POST") {
    // !!NOT COMPLETE!!
    await delay(1000);
    const { name, comment } = req.body;
    const challenges = JSON.parse(
      fs.readFileSync("sample/challenges.json", "utf8")
    );
    const id = randomUUID();
    const date = new Date();
    const newChallenge = {
      id,
      name,
      comment,
      createdAt: date,
      updatedAt: date,
    };
    challenges.challenges.push(newChallenge);
    fs.writeFileSync("sample/challenges.json", JSON.stringify(challenges));
    res.status(201).json(newChallenge);
  } else {
    res.status(405).end();
  }
}

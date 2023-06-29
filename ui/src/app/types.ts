export type Challenge = {
  id: string;
  name: string;
  tags: string[];
  points: number;
  solved: number;
  comment: Comment;
  createdAt: string;
  updatedAt: string;
};

export type Comment = {
  id: string;
  challengeId: string;
  comment: string;
  author: Author;
  url: string;
};

type Author = {
  name: string;
};

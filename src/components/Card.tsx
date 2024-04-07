import Link from "next/link";

interface Props {
  title: string;
  body: string;
  link: string;
}

export function Card({ title, body, link }: Props) {
  return (
    <Link
      className="card w-full bg-base-200 hover:scale-102 hover:border-blue-500 hover:shadow-blue-500 hover:shadow-sm border-2 border-transparent transition-border duration-300"
      href={link}
    >
      <div className="card-body text-center">
        <h2 className="text-2xl font-bold mb-2 text-center">{title}</h2>
        <p>{body}</p>
      </div>
    </Link>
  );
}

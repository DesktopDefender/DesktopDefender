import { open } from "@tauri-apps/api/shell";
import Link from "next/link";

interface Props {
  className: string;
  children: string | JSX.Element | JSX.Element[];
  url: string;
}

export default function ExternalLink({ className, children, url }: Props) {
  return (
    <Link
      className={className}
      href={""}
      onClick={(e) => {
        e.preventDefault();
        open(url);
      }}
    >
      {children}
    </Link>
  );
}

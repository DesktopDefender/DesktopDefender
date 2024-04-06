interface Props {
  className?: string;
  children: string;
}

export default function DDText({ className = "", children }: Props) {
  return <p className={className}>{children}</p>;
}

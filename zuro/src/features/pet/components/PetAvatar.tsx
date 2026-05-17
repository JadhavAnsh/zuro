interface PetAvatarProps {
  isAwake: boolean;
}

export function PetAvatar({ isAwake }: PetAvatarProps): React.JSX.Element {
  return (
    <span className="pet-avatar" aria-hidden="true" data-awake={isAwake}>
      <span className="pet-avatar__halo" />
      <span className="pet-avatar__body" />
      <span className="pet-avatar__eye pet-avatar__eye--left" />
      <span className="pet-avatar__eye pet-avatar__eye--right" />
      <span className="pet-avatar__spark pet-avatar__spark--one" />
      <span className="pet-avatar__spark pet-avatar__spark--two" />
    </span>
  );
}

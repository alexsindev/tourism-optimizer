export const DAY_COLORS = [
  "#3B82F6", // blue
  "#10B981", // green
  "#F59E0B", // amber
  "#EF4444", // red
  "#8B5CF6", // purple
  "#EC4899", // pink
  "#14B8A6", // teal
];

export const getDayColor = (day: number): string => {
  return DAY_COLORS[(day - 1) % DAY_COLORS.length];
};

export const CATEGORY_ICONS: Record<string, string> = {
  Museum: "🏛️",
  Restaurant: "🍽️",
  Landmark: "🗿",
  Park: "🌳",
  Shopping: "🛍️",
  Entertainment: "🎭",
};

export const getCategoryIcon = (category: string): string => {
  return CATEGORY_ICONS[category] || "📍";
};

export const formatTime = (minutes: number): string => {
  const hours = Math.floor(minutes / 60);
  const mins = minutes % 60;
  return `${hours.toString().padStart(2, "0")}:${mins.toString().padStart(2, "0")}`;
};

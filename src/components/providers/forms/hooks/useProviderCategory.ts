import { useEffect, useState } from "react";
import type { ProviderCategory } from "@/types";

interface UseProviderCategoryProps {
  selectedPresetId: string | null;
  isEditMode: boolean;
  initialCategory?: ProviderCategory;
}

export function useProviderCategory({
  selectedPresetId,
  isEditMode,
  initialCategory,
}: UseProviderCategoryProps) {
  const [category, setCategory] = useState<ProviderCategory | undefined>(
    isEditMode ? initialCategory : undefined,
  );

  useEffect(() => {
    if (isEditMode) {
      setCategory(initialCategory);
      return;
    }

    setCategory(selectedPresetId === "custom" ? "custom" : undefined);
  }, [selectedPresetId, isEditMode, initialCategory]);

  return { category, setCategory };
}

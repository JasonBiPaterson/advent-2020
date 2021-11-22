(ns advent-2020.day-03
  (:require [clojure.java.io :as io]
            [clojure.string :as string]))

(def input
  (-> "advent_2020/day_03/input"
      io/resource
      slurp
      string/split-lines))

(def width (count (first input)))

(defn tree-count [x y]
  (let [rows (keep-indexed (fn [idx row]
                             (when (zero? (mod idx y))
                               row))
                           input)
        chars (map-indexed (fn [idx row] (nth row (mod (* idx x) width) nil))
                           rows)]
    (-> (group-by identity chars)
        (get \#)
        count)))

(def part-1 (tree-count 3 1))

(def slopes [[1 1] [3 1] [5 1] [7 1] [1 2]])

(def part-2
  (->> slopes
       (map #(apply tree-count %))
       (reduce *)))

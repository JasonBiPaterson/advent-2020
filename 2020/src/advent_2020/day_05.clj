(ns advent-2020.day-05
    (:require [clojure.java.io :as io]
              [clojure.string :as string]))

(def input
  (-> "advent_2020/day_05/input"
      io/resource
      slurp
      string/split-lines))

(defn halfway [a b]
    (quot (+ a b) 2))

(defn binary-partition [char min-row max-row min-col max-col]
    (case char
        \F [min-row (halfway min-row max-row) min-col max-col]
        \B [(halfway min-row max-row) max-row min-col max-col]
        \L [min-row max-row min-col (halfway min-col max-col)]
        \R [min-row max-row (halfway min-col max-col) max-col]))

(def seat-ids
    (-> (for [line input
              :let [[_ row _ col] (reduce
                                   (fn [ranges char]
                                       (apply binary-partition char ranges))
                                   [0 127 0 7]
                                   line)]]
            (+ (* 8 row) col))
        sort
        vec))

(def part-1 (last seat-ids))

(def part-2
    (first
     (keep-indexed
      (fn [idx id]
          (when (= (get seat-ids (inc idx))
                   (+ 2 id))
              (inc id)))
      seat-ids)))

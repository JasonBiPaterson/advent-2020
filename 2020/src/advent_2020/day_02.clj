(ns advent-2020.day-02
  (:require [clojure.java.io :as io]
            [clojure.string :as string]))


(defn process-input [i]
  (for [line i]
    (let [[char-range [char] password] (string/split line #"\s")
          [a b] (mapv #(Integer/parseInt %) (string/split char-range #"-"))]
      [a b char password])))

(def input
  (-> "advent_2020/day_02/input"
      io/resource
      slurp
      string/split-lines
      process-input))

(defn get-count [pred]
  (->> input
       (filter #(apply pred %))
       count))

(def part-1
  (get-count (fn [minimum maximum char password]
               (let [char-count (count (filter #{char} password))]
                 (<= minimum char-count maximum)))))

(def part-2
  (get-count (fn [a b char password]
               (let [match1? (= char (nth password (dec a)))
                     match2? (= char (nth password (dec b)))]
                 (and (or match1? match2?)
                      (not (and match1? match2?)))))))

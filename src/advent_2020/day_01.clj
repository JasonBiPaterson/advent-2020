(ns advent-2020.day-01
  (:require [clojure.edn :as edn]
            [clojure.java.io :as io]))

(def input
  (edn/read-string (str "["
                        (-> "advent_2020/day_01/input" io/resource slurp)
                        "]")))

(defn multiply-matches [f n coll]
  (->> coll
        (map-indexed (fn [idx v]
                       (let [before (take idx coll)
                             after (drop (inc idx) coll)]
                         (some-> (f (- n v) (concat before after))
                                 (* v)))))
        (filter some?)
        first))

(defn find-pair [n coll]
  (some #{n} coll))

(defn find-triple [n coll]
  (multiply-matches find-pair n coll))

(def part-1
  (multiply-matches find-pair 2020 input))

(def part-2
  (multiply-matches find-triple 2020 input))

(defn run [_]
  (println part-1)
  (println part-2))

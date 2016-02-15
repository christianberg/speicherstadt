(ns speicherstadt.endpoint.blobs
  (:require [compojure.core :refer :all]))

(def target-size (* 4096 16))
(def buffer-size (* 2 target-size))

(defn chunk-stats [chunks]
  (let [size (reduce + chunks)
        cnt  (count chunks)]
    (str "Total size:         " size "\n"
         "Number of chunks:   " cnt "\n"
         "Average chunk size: " (double (/ size cnt)) "\n"
         "Min chunk size:     " (reduce min chunks) "\n"
         "Max chunk size:     " (reduce max chunks) "\n"
         "Chunks:             " (vec (take 10 chunks)) "\n")))

(defn splice [stream]
  (loop [total-size 0
         chunk-size 0
         buffer (vec (repeat buffer-size 0))
         position 0
         sum 0
         chunks []]
    (let [next-byte (.read stream)]
      (if (= next-byte -1)
        (conj chunks chunk-size)
        (let [sum (+ (- sum (buffer position)) next-byte)]
          (if (and (zero? (mod sum target-size))
                   (> chunk-size 32768))
            (recur (inc total-size)
                   0
                   (assoc buffer position next-byte)
                   (mod (inc position) buffer-size)
                   sum
                   (conj chunks (inc chunk-size)))
            (recur (inc total-size)
                   (inc chunk-size)
                   (assoc buffer position next-byte)
                   (mod (inc position) buffer-size)
                   sum
                   chunks)))))))

(defn blobs-endpoint [config]
  (routes
   (context "/blobs" []
            (POST "/" []
                  (fn [{:keys [body]}]
                    (-> (splice body)
                        chunk-stats))))))
